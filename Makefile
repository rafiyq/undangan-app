export PATH := $(HOME)/.global_modules/bin:$(PATH)

dev:
	tailwindcss -i ./static/input.css -o ./static/style.css --minify
	wrangler kv key put style.css --path static/style.css --binding static --preview
	wrangler dev --remote

publish:
	tailwindcss -i ./static/input.css -o ./static/style.css --minify
	wrangler kv key put style.css --path static/style.css --binding static --preview false
	wrangler deploy

clean:
	rm -rf target
	rm -rf node_modules
	rm -rf .wrangler
	rm -rf build