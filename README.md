# Germinate CLI

This is a CLI implementation of the [germinate](https://github.com/itmecho/germinate) library. It is meant for templating files before a service starts up, for example a systemd unit `ExecPreStart` command or as part of a docker container startup process.

## Example Usage

```
germinate /etc/fluentd/fluentd.conf.tmpl /etc/fluentd/fluentd.conf
```

# License

[GPL-3.0](https://github.com/itmecho/germinate-cli/blob/master/LICENSE)
