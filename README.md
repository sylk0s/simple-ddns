# Simple DDNS

A rewrite of [sylk0s/cloudflare-ddns](https://github.com/sylk0s/cloudflare-ddns).

### Rationale:
1. I wanted to write the project in rust
2. cloudflare-ddns was unreliable and hard to maintain

Also, the rust clodflare library is still in development, and has little documentation, so I decided to just use `reqwest` to make http calls instead to their API since I'm just trying to do a hyperspecific thing.

## Usage

### Docker
Put your cloudflare token in a file called `./token`. Modify the script to include your ZONE_NAME and RECORD_NAME. Then run `./build_docker.sh`.

### Nix
For now, a shell.nix is provided for rust development on nix. Otherwise, follow the Development guide
> TODO: Implement building and deployment in nix automatically

### Development
Use a `.env` file to specifiy `ZONE_NAME`, `RECORD_NAME`, and `TOKEN`. This will be loaded into the enviornment variables when run.
