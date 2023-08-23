# Spin Plugin: Check-for-Update

Using the `check-for-update` plugin, you can quickly check if your Spin CLI installation is outdated or not. It compares the locally installed version of `spin` with the latest (non-canary) release available.

If the local version of `spin` differs from the most recent release, a link with detailed installation instructions will be printed to `stdout`.

## Backend

The backend for this plugin is deployed to Fermyon Cloud and responsible for querying the most recent stable release number of `spin` CLI. The source code for the backend is available at https://github.com/ThorstenHans/get-latest-spin-cli-version
