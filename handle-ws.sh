#!/bin/bash

tail -F /tmp/foo.out &
cat -u >> /tmp/foo.in
