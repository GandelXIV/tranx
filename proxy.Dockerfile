FROM nginx:1.25-alpine

COPY www /srv/www

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 8000
