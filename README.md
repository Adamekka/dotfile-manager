# dotfile-manager

## Running from a container

### 1. Build an image

From the root of the project run:

```bash
docker build -t "dman" .
```

### 2. Run the image

```bash
docker run -it dman
```

This will open a shell session in the container's root user's home directory.

### 3. Run commands

You can now run `dman` or `dman-gui` commands.
