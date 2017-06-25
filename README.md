CaronaBoard Weather Checker
===========================

This is a small rust script that checks the weather and send an email
if it is raining. To run it, you just need docker.

First, you have to build it:

```bash
make build
```

Set your env variables properly:

```bash
export API_KEY='open weather api key'
export EMAIL_TO='email destination'
export SMTP_PASS='mailgun smtp pass'
```

Then run it:

```bash
make run
```

For development mode, you can use `cargo run` instead of make, to run it
outside docker.

## Deploy

The deploy is currently manual, to heroku. To run it:

```bash
heroku plugins:install heroku-container-registry
heroku container:login
heroku container:push web
```

That's it!
