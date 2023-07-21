<template>
    <td class="">
        {{ contacto.id }}
    </td>
    <td>{{ contacto.nombre }}</td>
    <td>{{ contacto.telefono }}</td>
    <td>{{ contacto.email }}</td>
    <td><button>EDITAR</button></td>
</template>
<script setup>
const props = defineProps({
    contacto: Object
})
</script>