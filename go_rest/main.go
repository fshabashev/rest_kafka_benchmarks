package main

import (
	"fmt"
	"github.com/confluentinc/confluent-kafka-go/kafka"
	"io"
	"net/http"
)

func kafka_ingest(producer *kafka.Producer, value string) {
	// Create a new producer instance
	// p, err := kafka.NewProducer(&kafka.ConfigMap{"bootstrap.servers": "localhost:9092"})

	// Defer closing the producer to ensure all messages are sent

	// Produce a message to the "test" topic
	topic := "test"
	err := producer.Produce(&kafka.Message{
		TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
		Value:          []byte(value),
	}, nil)

	if err != nil {
		fmt.Printf("Error producing message: %v\n", err)
	}
}

func main() {
	// Create a new producer instance
	producer, err := kafka.NewProducer(&kafka.ConfigMap{"bootstrap.servers": "localhost:29092"})
	// defer producer.Close()
	if err != nil {
		fmt.Printf("Error connecting to kafka")
	}

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			// Read the request body
			body := make([]byte, r.ContentLength)
			_, err := r.Body.Read(body)
			if (err != io.EOF) && (err != nil) {
				http.Error(w, "Error reading request body", http.StatusBadRequest)
				return
			}
			kafka_ingest(producer, "hello from golang")

			// Return the request body in the response
			w.Write(body)
		} else {
			http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		}
	})

	fmt.Println("Starting server on port 8080")
	http.ListenAndServe(":8080", nil)
}
