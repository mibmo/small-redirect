# small-redirect
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mibmo/small-redirect/Build%20and%20publish%20container?style=plastic)
A lightweight HTTP redirect container

# Configuration
small-redirect is configured through environment variables.
There are only two at the moment:
- `ADDR`: the address for the HTTP server to listen on. _(note: disabled when using Docker feature)_
- `REDIRECT_URI`: address to redirect to.

# Container
[![Build and publish container](https://github.com/mibmo/small-redirect/actions/workflows/build-image.yaml/badge.svg)](https://github.com/mibmo/small-redirect/actions/workflows/build-image.yaml)

A container image is continually built upon release and available on the [GitHub Container Registry](ghcr.io/mibmo/small-redirect).
The latest image can be pulled with `docker pull ghcr.io/mibmo/small-redirect:latest`
