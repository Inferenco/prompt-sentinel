# Build stage
FROM node:20-alpine AS build

WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install

# Copy source files
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM node:20-alpine AS production

WORKDIR /app

# Copy built files from build stage
COPY --from=build /app/dist ./dist
COPY --from=build /app/package*.json ./

# Install dependencies including vite for preview
RUN npm install

# Expose the port the app runs on
EXPOSE ${FRONTEND_PORT:-5175}

# Command to run the application
CMD ["sh", "-c", "npm run preview -- --port ${FRONTEND_PORT:-5175} --host 0.0.0.0"]
