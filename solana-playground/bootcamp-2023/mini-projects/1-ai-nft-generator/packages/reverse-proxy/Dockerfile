FROM nginx:1.27.3-alpine3.20-perl

RUN apk add --no-cache gettext certbot python3-certbot-nginx

RUN certbot --nginx -d nft-ai-generator.undo.it
RUN certbot renew --dry-run

WORKDIR /usr/src/app
COPY . .
EXPOSE 80

RUN chmod +x ./scripts/entrypoint.sh && \
    chmod +x ./scripts/substitute-environment-variables.sh

CMD [ "/usr/src/app/scripts/entrypoint.sh" ]
