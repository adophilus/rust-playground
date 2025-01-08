FROM nginx:stable-alpine-slim

WORKDIR /usr/src/app
COPY .
EXPOSE 80

CMD [ "nginx", "-c"  "/usr/src/app/nginx.conf"]
