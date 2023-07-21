<script setup>
import { createUserWithEmailAndPassword } from "firebase/auth";
import { auth } from '../server/firebase';

const email = ref()
const password = ref()
const user = ref()

const create = async () => {
    createUserWithEmailAndPassword(auth, email.value, password.value)
        .then((userCredential) => {
            return user.value = userCredential.user
        })
        .catch(error => {
            const errorCode = error.code
            const errorMessage = error.message
        })
}

</script>
<template>
    <div class="w-full max-w-xs">
        <form @submit.prevent="create()" class="shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <div class="mb-4">
                <input type="email" v-model="email" class="shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline">
            </div>
            <div class="mb-6">
                <input type="password" v-model="password" class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline">
            </div>
            <div class="flex items-center justify-between">
                <button type="submit" class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">Crear Usuario</button>
            </div>
        </form>
        <span v-if="user">{{ user }}</span>
    </div>
</template>