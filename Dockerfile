# Use an official Maven image as the base image
FROM maven:3.9.5-amazoncorretto-21 as build
# Set the working directory in the container
WORKDIR /app
# Copy the pom.xml and the project files to the container
COPY pom.xml .
COPY src ./src
# Build the application using Maven
RUN mvn clean package -DskipTests

# Use an official OpenJDK image as the base image
FROM amazoncorretto:21-alpine3.18
# Set the working directory in the container
WORKDIR /app
# Copy the built JAR file from the previous stage to the container
COPY --from=build /app/target/sexo-1.0-SNAPSHOT-jar-with-dependencies.jar .
# Set the command to run the application
CMD ["java", "-jar", "sexo-1.0-SNAPSHOT-jar-with-dependencies.jar"]
