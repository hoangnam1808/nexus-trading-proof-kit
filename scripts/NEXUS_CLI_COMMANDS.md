# Nexus CLI Commands

Recommended on Windows: use WSL Ubuntu, not native PowerShell, for the shell install command.

```bash
curl https://cli.nexus.xyz/ | sh
source ~/.bashrc
nexus-network register-user --wallet-address <your-wallet-address>
nexus-network register-node
nexus-network start
```

Start again later with a known node ID:

```bash
nexus-network start --node-id <your-node-id>
```

Logout / clear credentials:

```bash
nexus-network logout
```

Never paste seed phrase or private key into terminal commands.
