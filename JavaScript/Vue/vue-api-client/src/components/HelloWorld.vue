<script setup>
import { ref } from "vue";
import DataRow from "./DataRow.vue";

const loading = ref(false);

// 
// Respuesta
// 
const Respuesta = ref('[{"mensaje":"hola"}]')

// 
// API
// 
const ENDPOINT_API = import.meta.env.VITE_API_ENDPOINT;

// 
// Datos de ejemplo
// 
const POST_EXAMPLE = {
  id: 0,
  nombre: "Steban Hincapie",
  telefono: "999",
  email: "a@a.a"
}

const DELETE_EXAMPLE = {
  id: 0
}

const PUT_EXAMPLE = {
  id: 0,
  nombre: "Steban Hincapie",
  telefono: "999",
  email: "a@a.a"
}

// 
// Uso de funciones
// 
getData(ENDPOINT_API)


// 
// FUNCIONES
// 
async function getData(url = '') {
  loading.value = false
  const response = await fetch(url, {
    method: 'GET',
  });
  Respuesta.value = await response.json()
  return loading.value = true;
}

async function postData(url = '', data = {}) {
  return fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  }).then(response => response.json());
}

async function putData(url = '', data = {}) {
  return fetch(url, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  }).then(response => {
    response.json()
    console.log(response);
    Respuesta.value = response
  });
}

async function deleteData(url = '') {
  return fetch(url, {
    method: 'DELETE',
  }).then(response => response.json());
}

</script>
<template>
  <div class="">
    <button v-on:click="getData(ENDPOINT_API)" class="my-2">Refrescar</button>
    <table class="table-auto">
      <thead>
        <tr>
          <th>ID</th>
          <th>NOMBRE</th>
          <th>TELEFONO</th>
          <th>EMAIL</th>
          <th>ACCIONES</th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="loading" v-for="contacto in Respuesta" :key="contacto.id">
          <DataRow :contacto="contacto"></DataRow>
        </tr>
      </tbody>
    </table>
  </div>
</template>