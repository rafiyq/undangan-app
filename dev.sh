#!/bin/bash

tailwindcss -i ./static/tailwind.css -o ./static/style.css --minify && \
wrangler kv key put style.css --path static/style.css --binding assets --preview && \
wrangler dev --remote