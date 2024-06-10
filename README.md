# F3 slack bot

## Deployment process

### Before pushing master

1. Make sure to run sqlx local snapshot

```
cargo sqlx prepare
```

2. Run clippy and tests

```shell
cargo clippy && cargo test
```

3. Make sure in DigitalOcean db is open to external connection.
   You will need this security disabled in order to run migrations if there are any.

4. Push to deploy and watch deployment in DigitalOcean.

### After deployment is successful

1. If any migrations, run the following

```shell
DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```

2. Also run sync method

```
https://f3boiseapi-cycjv.ondigitalocean.app/sync
```
