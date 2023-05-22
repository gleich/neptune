convert-png:
	sips -s format jpeg assets/$(filename).png --out assets/$(filename).jpg
	rm assets/$(filename).png