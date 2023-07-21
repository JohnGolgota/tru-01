package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"github.com/joho/godotenv"
)

type User struct {
	ID       int    `json:"id"`
	Nombre   string `json:"nombre"`
	Telefono string `json:"telefono"`
	Email    string `json:"email"`
}

func main() {
	err := godotenv.Load()
	if err != nil {
		fmt.Println("Error al cargar el archivo .env:", err)
		return
	}

	url := os.Getenv("API_ENDPOINT")
	// pqrs := getPQRS(url)

	// fmt.Println(pqrs)

	data := []byte(`{
		"estado": "GO",
		"fecha_registro": "HOY",
		"tipo_doc": "Normal",
		"identificacion": "Una",
		"nombres_apellidos": "Cuatro",
		"email": "si tengo",
		"tel": "tambien",
		"mensaje": "hola mundo",
		"observaciones": "xd",
		"gdpr": "true",
	}`)

	// Crear una solicitud POST con los datos
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(data))
	if err != nil {
		fmt.Println("Error al crear la solicitud:", err)
		return
	}

	// Opcionalmente, puedes agregar encabezados personalizados a la solicitud
	req.Header.Set("Content-Type", "application/json")

	// Realizar la solicitud HTTP
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error al enviar la solicitud:", err)
		return
	}
	defer resp.Body.Close()

	// Leer la respuesta del servidor
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error al leer la respuesta:", err)
		return
	}

	// Imprimir la respuesta del servidor
	fmt.Println("Respuesta:", string(body))

}

func getPQRS(apiEnpoint string) []User {
	// Solicitud
	resp, err := http.Get(apiEnpoint)
	if err != nil {
		fmt.Println("Error al realizar la solicitud", err)
		return []User{}
	}
	defer resp.Body.Close()

	// Leer respuesta
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error al leer la respuesta:", err)
		return []User{}
	}

	var users []User
	err = json.Unmarshal([]byte(body), &users)
	if err != nil {
		fmt.Println("Error al analizar JSON", err)
		return []User{}
	}

	for _, user := range users {
		fmt.Println("ID:", user.ID)
		fmt.Println("Nombre:", user.Nombre)
		fmt.Println("Telefono:", user.Telefono)
		fmt.Println("Email:", user.Email)
	}

	prettyJSON, err := json.MarshalIndent(users, "", "  ")
	if err != nil {
		fmt.Println("Error al convertir a JSON", err)
		return []User{}
	}

	fmt.Println(string(prettyJSON))
	return users
}
