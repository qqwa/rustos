#!/bin/sh
rustdoc "$@" --document-private-items
# rustdoc --no-defaults --passes collapse-docs --passes unindent-comments "$@"
