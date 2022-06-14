FROM node:18.3.0-alpine

WORKDIR /src
COPY . ./
RUN ls -lh
RUN npm install --location=global @angular/cli
RUN npm install
EXPOSE 4200
CMD ["ng", "serve", "--host", "0.0.0.0", "--poll", "1000", "--port", "4200", "--disable-host-check"]


