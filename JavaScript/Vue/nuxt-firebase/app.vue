<template>
  <div>
    <nav class="flex">
        <NuxtLink to="/singin" class="p-3 bg-slate-400 text-white hover:bg-white hover:text-slate-400">crear</NuxtLink>
        <NuxtLink to="/login" class="p-3 bg-slate-400 text-white hover:bg-white hover:text-slate-400">iniciar</NuxtLink>
        <NuxtLink to="/loged" class="p-3 bg-slate-400 text-white hover:bg-white hover:text-slate-400">privado</NuxtLink>
    </nav>
    <NuxtPage />
  </div>
</template>
<script setup lang="ts">
const config = useRuntimeConfig()

</script>