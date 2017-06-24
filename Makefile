build:
	docker build -t weather-checker .

run:
	docker run -i -e API_KEY=$$API_KEY -e EMAIL_TO=$$EMAIL_TO -e SMTP_PASS=$$SMTP_PASS -t weather-checker ./target/release/weather-checker
