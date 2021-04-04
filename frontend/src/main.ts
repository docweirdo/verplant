import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import PrimeVue from "primevue/config";

import "primevue/resources/themes/md-light-indigo/theme.css" //theme
import "primevue/resources/primevue.min.css"                 //core css
import "primeicons/primeicons.css"                           //icons
import moment from "moment";


const app = createApp(App);


app.use(router);
app.use(PrimeVue, {ripple: true});

moment.locale('de');

app.mount("#app");
