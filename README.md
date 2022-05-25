# mmtrigger

Excute a command when memory used passes a certain threshold.

## Usage

```
Usage: mmtrigger [options]

Options:
    -h, --help          print this help menu
    -m, --memory THRESHOLD
                        memory threshold
    -c, --command COMMAND
                        shell command
```

## Example

```bash
mmtrigger -m 80 -c 'kill -INT $(ps aux | grep something | cut -f 4 -d " ")'
```