<script setup lang="ts">
const members = await useMembers();
</script>

<template>
    <div v-for="member in members">
        <div>
            <img :src="member.picture" :alt="member.name" />
        </div>

        <div>
            <div>
                <h3>{{ member.name }}</h3>
                <span v-if="member.verifiedAt"> Verified </span>
            </div>

            <div>
                <div>
                    <span>E-mail</span>
                    <span>{{ member.email }}</span>
                </div>
                <div>
                    <span>Phone number</span>
                    <span>{{ member.phoneNumber }}</span>
                </div>
                <div>
                    <span>Birthdate</span>
                    <span>{{ member.birthdate }}</span>
                </div>
                <div>
                    <span>Gender</span>
                    <span>{{ member.gender }}</span>
                </div>
            </div>
        </div>
    </div>
</template>
