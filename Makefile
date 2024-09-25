dev:
	tailwindcss -i ./static/tailwind.css -o ./static/style.css --minify
	wrangler kv key put style.css --path static/style.css --binding assets --preview
	wrangler dev --remote

publish:
	tailwindcss -i ./static/tailwind.css -o ./static/style.css --minify
	wrangler kv key put style.css --path static/style.css --binding assets --preview false
	wrangler deploy

clean:
	rm -rf target
	rm -rf node_modules
	rm -rf .wrangler
	rm -rf build