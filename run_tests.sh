#!/bin/bash

multirust run stable cargo test
multirust run beta cargo test
multirust run nightly cargo test
multirust run nightly cargo test --features nightly
