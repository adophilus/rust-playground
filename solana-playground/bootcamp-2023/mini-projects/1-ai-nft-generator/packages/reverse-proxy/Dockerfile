FROM nginx:1.27.3-alpine3.20-perl

RUN apk add --no-cache gettext

WORKDIR /usr/src/app
COPY . .
EXPOSE 80

RUN chmod +x entrypoint.sh

CMD [ "/usr/src/app/entrypoint.sh" ]
