FROM node:18


# Create app directory
WORKDIR /app
# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
# RUN npm cache clean --force
COPY package*.json ./
RUN cargo run
# RUN npm install --check-files
# If you are building your code for production
# RUN npm ci --only=production

# Bundle app source
COPY . .
CMD [ "cargo", "run" ]
EXPOSE 8080/tcp