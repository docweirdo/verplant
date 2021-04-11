<template>
  <div class="login">
    <div class="login-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.loginCardTitle }}
      </h1>
      <Card class="center-card">
        <template #content>
          <div class="login-fields">
            <label for="email-field">{{ currentTranslation.email }}</label>
            <InputText type="email" id="email-field" v-model="email" />
            <label for="password-field">{{
              currentTranslation.password
            }}</label>
            <InputText type="password" id="password-field" v-model="password" />
          </div>
        </template>
      </Card>
      <div class="login-container">
        <Button :label="currentTranslation.login" id="login-button" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// Foreign stuff
import { defineComponent, ref } from "vue";

// Our stuff
import { api, Course } from "@/api";
import { currentTranslation } from "@/translations";

// Foreign Components
import Card from "primevue/card";
import InputText from "primevue/inputtext";
import Button from "primevue/button";

// Our Components

export default defineComponent({
  name: "Login",
  components: {
    Card,
    InputText,
    Button,
  },
  setup() {
    const email = ref(null);

    return {
      currentTranslation,
      email,
    };
  },
});
</script>

<style scoped>
.login {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: var(--surface-800);
  background-image: url("../assets/splash01.svg");
  background-repeat: no-repeat;
  background-position: center;
  background-size: 100%;
}

#card-headline {
  font-size: 2em;
  font-weight: 500;
  color: white;
  margin-bottom: 5px;
}

@media only screen and (max-height: 900px) {
  #card-headline {
    font-size: 1.2em;
  }
}

.center-card {
  padding-top: 0px;
  margin: 0 auto;
}

.login-fields {
  display: flex;
  flex-direction: column;
}

.p-inputtext {
  width: 300px;
}

.login-fields > label {
  display: inline-block;
  margin-bottom: 0.2em;
  margin-top: 1em;
}

.login-container {
  width: 100%;
  display: flex;
  justify-content: end;
}

.login-container > #login-button {
  margin-top: 0.5em;
  background-color: white;
  color: var(--accentColor);
}
</style>