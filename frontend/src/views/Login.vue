<template>
  <div class="login">
    <div class="login-content">
      <h1 id="card-headline" class="p-component">
        {{ currentTranslation.loginCardTitle }}
      </h1>
      <Card class="center-card">
        <template #content>
          <div class="login-fields">
            <template v-if="error">
              <label for="email-field">{{ currentTranslation.email }}.</label>
              <InputText
                type="email"
                id="email-field"
                v-model="email"
                class="p-invalid"
              />
              <label for="password-field">{{
                currentTranslation.password
              }}</label>
              <InputText
                type="password"
                id="password-field"
                v-model="password"
                class="p-invalid"
                v-on:keyup.enter="login"
              />
              <small id="login-failed" class="p-error"
                >{{ currentTranslation.loginError }}.</small
              >
            </template>
            <template v-else>
              <label for="email-field">{{ currentTranslation.email }}</label>
              <InputText type="email" id="email-field" v-model="email" />
              <label for="password-field">{{
                currentTranslation.password
              }}</label>
              <InputText
                type="password"
                id="password-field"
                v-model="password"
                v-on:keyup.enter="login"
              />
            </template>
          </div>
        </template>
      </Card>
      <div class="login-container">
        <Button
          :label="loading ? '' : currentTranslation.login"
          id="login-button"
          @click="login"
          :icon="loading ? 'pi pi-spin pi-spinner' : ''"
          :class="{ loading: loading }"
        />
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
import router from "@/router";

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
    const email = ref("");
    const password = ref("");
    const error = ref(false);
    const loading = ref(false);

    const login = async () => {
      loading.value = true;
      // email validation, etc

      // TODO: remove fake delay
      await new Promise((r) => setTimeout(r, 2000));

      try {
        if (!(await api.login(email.value, password.value))) {
          error.value = true;
          // do something else?
        } else {
          error.value = false;
          // redirect to base provider site
          // if path is /login?redirect=/to/page, redirects to this path
          const redirect = router.currentRoute.value.query.redirect;
          if (redirect && !Array.isArray(redirect)) {
            await router.replace(redirect);
            // if this fails, we catch the error below
          } else {
            router.replace("/overview");
          }
          console.log("redirected");
        }
      } catch (error) {
        console.warn(error);
      } finally {
        loading.value = false;
      }
    };

    return {
      currentTranslation,
      email,
      password,
      login,
      error,
      loading,
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
  justify-content: flex-end;
}

.login-container > #login-button {
  margin-top: 0.5em;
  background-color: white;
  color: var(--accentColor);
  width: 150px;
  transition: width 0.3s ease-in-out;
}

.login-container > #login-button.loading {
  width: 50px;
}

.login-content .p-error {
  margin-top: 0.5em;
}
</style>

<style>
.center-card .p-card-body {
  padding-bottom: 8px;
}
</style>