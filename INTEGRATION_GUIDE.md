# Region Integration Implementation Guide

## What We Built

A simple, extensible system for syncing preblasts from the F3 API to external integrations (starting with F3 Boise Slack).

## Architecture

```
F3 UI → F3 API → Check Region Integration → Slack Bot API → Slack
```

### Database Changes

**F3 API** (`/f3` repo):
- ✅ Migration: `migrations/20260416223305_region_integrations_column.sql`
  - Added `integration_type` column to `regions` table
  - Valid values: `'f3_boise_slack'` or `NULL`

### Backend Changes

**F3 API** (`/f3` repo):

1. **Database Layer** (`libs/db/rs_db/src/`):
   - ✅ Updated `regions/mod.rs`:
     - Added `integration_type` field to `RegionEntry` struct
     - Updated all region queries to include `integration_type`
     - Added `get_integration_type_for_ao()` helper

   - ✅ Updated `users/user_info.rs`:
     - Added `UserIntegrationData` struct
     - Added `get_user_integration_data()` to get slack_ids + F3 names

   - ✅ Updated `equipment/mod.rs`:
     - Added `get_equipment_names_by_ids()` helper

   - ✅ Created `integrations/mod.rs`:
     - `sync_preblast_to_integration()` - main integration router
     - `sync_to_f3_boise_slack()` - Slack-specific implementation
     - Fire-and-forget HTTP call to Slack Bot API

2. **API Routes** (`libs/shell/rs_api_routes/src/`):
   - ✅ Updated `pre_blasts.rs`:
     - Added integration sync call after preblast creation

3. **Models** (`libs/shared/models_common/src/`):
   - ✅ Updated `Region` struct to include `integration_type`

**Slack Bot API** (this repo - `/f3-scraper-rs`):

1. **Routes**:
   - ✅ Created `src/web_api_routes/pre_blast_data/external.rs`
     - New endpoint: `POST /pre_blasts/from-external`
     - Accepts preblast data from F3 API
     - Matches AO by name (case-insensitive)
     - Handles users with/without Slack IDs
     - Posts to Slack with proper formatting

   - ✅ Updated `src/web_api_run/pre_blasts.rs`:
     - Registered new route

## Data Flow

### Request Payload (F3 API → Slack Bot)

The F3 API sends this JSON when a preblast is created in a region with `integration_type = 'f3_boise_slack'`:

```json
{
  "ao_name": "Bleach",
  "title": "The Iron Pax Challenge",
  "date": "2026-04-20",
  "time": "05:30",
  "q_slack_ids": ["U12345", "U67890"],
  "q_names_without_slack": ["Crash Override"],
  "why": "We're testing the integration",
  "equipment": ["Coupons", "Pull-up bar"],
  "fng_message": "Bring a friend!",
  "description": "Extra details here"
}
```

### User Handling

The Slack Bot API handles two types of Q assignments:

1. **Users with Slack IDs**:
   - Received in `q_slack_ids` array
   - Displayed as mentions in Slack: `<@U12345>`
   - Stored in database with their F3 name (via slack_id lookup)

2. **Users without Slack IDs**:
   - Received in `q_names_without_slack` array
   - Displayed as plain text in Slack
   - Stored in database as-is

### AO Matching

- **Method**: Name-based matching (case-insensitive)
- **Process**:
  1. F3 API sends AO name (e.g., `"Bleach"`)
  2. Slack Bot converts to enum: `AO::from("Bleach")` → `AO::Bleach`
  3. Gets channel ID: `AO::Bleach.channel_id()` → `"C01ABC123"`
  4. Posts to that channel

- **Fallback**:
  - If AO not recognized → `AO::Unknown("Bleach")`
  - Returns `400 Bad Request`
  - Preblast NOT saved
  - Message: `"Unknown AO: Bleach. Skipping Slack post."`

### Equipment Matching

- **Process**:
  1. F3 API sends equipment names as strings (e.g., `["Coupons", "Pull-up bar"]`)
  2. Slack Bot attempts to match to `AoEquipment` enum
  3. If match fails → wraps as `AoEquipment::Other("custom name")`

## Configuration

### Environment Variables

This Slack Bot API needs to know it can accept requests from F3 API. No authentication currently (assumes internal network).

**Future**: Add API key authentication:
```bash
F3_API_KEY=your-secret-key-here
```

### F3 API Configuration

The F3 API needs this environment variable:

```bash
SLACK_BOT_API_URL=http://localhost:8080
# Or in production:
# SLACK_BOT_API_URL=https://slack-bot.f3boise.com
```

### Region Settings (F3 UI)

**Location**: Region Settings Page

**New Field**: Integration Type (Dropdown)
- Options:
  - `None` (NULL) - No integration
  - `F3 Boise Slack` (f3_boise_slack) - Sync to Boise Slack workspace

## Endpoint Documentation

### `POST /pre_blasts/from-external`

Create a preblast from an external source (F3 API) and post to Slack.

**Request Body**:
```typescript
{
  ao_name: string;                // AO name (e.g., "Bleach", "Gem")
  title: string;                  // Preblast title
  date: string;                   // YYYY-MM-DD format
  time: string;                   // HH:MM format (24-hour)
  q_slack_ids: string[];          // Slack user IDs (e.g., ["U12345"])
  q_names_without_slack: string[]; // F3 names for users without Slack IDs
  why?: string;                   // Optional: why/description
  equipment: string[];            // Equipment names
  fng_message?: string;           // Optional: message for FNGs
  description?: string;           // Optional: moleskine/additional notes
}
```

**Success Response** (200 OK):
```json
{
  "id": "uuid-here",
  "success": true,
  "message": "Preblast created and posted to Slack successfully"
}
```

**Error Responses**:

- `400 Bad Request` - Unknown AO:
  ```json
  {
    "id": "",
    "success": false,
    "message": "Unknown AO: XYZ. Skipping Slack post."
  }
  ```

- `400 Bad Request` - Invalid date/time:
  ```json
  {
    "id": "",
    "success": false,
    "message": "Invalid date format: ..."
  }
  ```

- `500 Internal Server Error` - Database error:
  ```json
  {
    "id": "",
    "success": false,
    "message": "Failed to save preblast: ..."
  }
  ```

## Testing

### 1. Set up Environment

**Slack Bot API** (this repo):
```bash
cd /Users/edwardtkachev/Public/repos/f3-scraper-rs
cargo run
# Server starts on http://localhost:8080
```

**F3 API**:
```bash
cd /Users/edwardtkachev/Public/repos/f3
export SLACK_BOT_API_URL=http://localhost:8080
cargo run
```

### 2. Enable Integration (F3 API database)

**Option A - Via SQL**:
```sql
UPDATE regions
SET integration_type = 'f3_boise_slack'
WHERE id = 1;  -- Replace with your region ID
```

**Option B - Via UI** (once UI is built):
- Go to Region Settings
- Select "F3 Boise Slack" from dropdown
- Save

### 3. Create a Test Preblast (F3 API)

**Via F3 UI or API**:
```bash
POST http://localhost:3000/pre_blasts/with-image
{
  "title": "Test Preblast",
  "blast_date": "2026-04-20",
  "blast_time": "05:30",
  "ao_id": 1,
  "qs": ["<user-uuid-with-slack-id>"],
  "why": "Testing the integration",
  "equipment_ids": [1, 2],
  "fng_message": "Welcome!",
  "description": "Test"
}
```

### 4. Verify Integration

**Check Logs**:

F3 API console:
```
Successfully synced preblast to Slack Bot
```

Slack Bot API console:
```
Received external preblast request: ExternalPreBlastRequest { ao_name: "Bleach", ... }
Successfully synced preblast to Slack Bot
```

**Check Slack**:
- Message appears in appropriate AO channel
- Q mentions work correctly (`<@U12345>`)
- Equipment displays properly
- Edit button works

**Check Databases**:
- F3 API database: Preblast exists with correct data
- Slack Bot database: Preblast exists with correct data

### 5. Test Error Cases

**Unknown AO**:
```bash
# In F3 database, create an AO with name that doesn't exist in Slack Bot
# Should get 400 error
```

**User without Slack ID**:
```bash
# Create preblast with Q who has no slack_id in F3 database
# Should appear as plain text in Slack, still save correctly
```

## Error Handling

### AO Not Found
- **HTTP Response**: `400 Bad Request`
- **Message**: `"Unknown AO: XYZ. Skipping Slack post."`
- **Action**: Preblast NOT saved, integration skipped
- **Reason**: Prevents posting to wrong channel or invalid data

### Slack Bot API Down / Network Error
- **Behavior**: Fire-and-forget from F3 API, logged to stderr
- **F3 API**: Preblast saved successfully
- **Slack Bot**: No record, no Slack post
- **Recovery**: Manual repost or wait for next preblast

### Invalid Date/Time Format
- **HTTP Response**: `400 Bad Request`
- **Message**: `"Invalid date format: ..."`
- **Action**: Preblast NOT saved

### Database Save Failure
- **HTTP Response**: `500 Internal Server Error`
- **Message**: `"Failed to save preblast: ..."`
- **Action**: Not posted to Slack

## Code Organization

### This Repo (Slack Bot API)

```
src/
  web_api_routes/
    pre_blast_data/
      mod.rs                    # Main preblast routes
      external.rs               # NEW: External integration endpoint
  web_api_run/
    pre_blasts.rs              # Route registration (updated)
  app_state/
    ao_data.rs                 # AO enum with channel mappings
    equipment.rs               # Equipment enum
    pre_blast_data.rs          # PreBlastData struct
  db/
    save_pre_blast.rs          # Database save logic
  slack_api/
    chat/post_message/         # Slack message posting
```

### Key Files

**`src/web_api_routes/pre_blast_data/external.rs`**:
- Main integration handler
- Parses external request
- Validates AO exists
- Handles users with/without Slack IDs
- Saves to database
- Posts to Slack

**`src/app_state/ao_data.rs`**:
- `AO` enum with all supported AOs
- `From<String>` trait for name matching
- `channel_id()` method for Slack channel mapping

## Future Enhancements

### Near-term (Slack Bot API side)
- [ ] API key authentication for external requests
- [ ] Support for editing preblasts via external API
- [ ] Support for deleting preblasts via external API
- [ ] Webhook to notify F3 API when Slack edits occur

### Later
- [ ] Backblast integration (similar pattern)
- [ ] Announcements integration
- [ ] Support for multiple Slack workspaces (multi-tenancy)
- [ ] Integration health monitoring endpoint
- [ ] Retry logic for failed Slack posts

## File Summary (This Repo)

### New Files
- `src/web_api_routes/pre_blast_data/external.rs` - External preblast endpoint
- `INTEGRATION_GUIDE.md` - This file

### Modified Files
- `src/web_api_routes/pre_blast_data/mod.rs` - Export external module
- `src/web_api_run/pre_blasts.rs` - Register `/from-external` route

## Troubleshooting

### Preblast not appearing in Slack

1. **Check F3 API logs**:
   - Look for "Successfully synced preblast to Slack Bot"
   - If missing, check if region has `integration_type = 'f3_boise_slack'`

2. **Check Slack Bot API logs**:
   - Look for "Received external preblast request"
   - If missing, check `SLACK_BOT_API_URL` in F3 API

3. **Check for AO mismatch**:
   - F3 API AO name must match Slack Bot AO enum name
   - Case-insensitive but must be exact match

4. **Check Slack permissions**:
   - Bot must be member of target channel
   - Bot must have `chat:write` permission

### Users not mentioned correctly

1. **Check slack_id field**:
   ```sql
   SELECT id, name, slack_id FROM users WHERE id = '<user-uuid>';
   ```

2. **Check Slack ID format**:
   - Should start with 'U' (e.g., `U12345ABC`)
   - If NULL, user appears as plain text (expected)

3. **Check F3 API integration data**:
   - `q_slack_ids` should contain valid Slack IDs
   - `q_names_without_slack` should contain F3 names

### Equipment not displaying

1. **Check equipment mapping**:
   - F3 API sends equipment names as strings
   - Slack Bot tries to match to `AoEquipment` enum
   - Unknown equipment wrapped as `AoEquipment::Other(...)`

2. **Check equipment_ids**:
   ```sql
   SELECT id, name FROM equipment WHERE id IN (1, 2, 3);
   ```

## Support

For issues or questions:
1. Check this guide first
2. Check F3 API integration documentation
3. Review error logs on both services
4. Test with minimal payload to isolate issue
