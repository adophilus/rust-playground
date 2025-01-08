FROM nginx:1.27.3-alpine3.20-perl

WORKDIR /usr/src/app
COPY . .
EXPOSE 80

CMD [ "nginx", "-c", "/usr/src/app/nginx.conf" ]
