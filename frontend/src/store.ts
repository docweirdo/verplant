import { reactive } from "@vue/reactivity";

class VerplantStore {
  contactInformations = reactive({
    firstname: "",
    lastname: "",
    email: "",
    phone: "",
    organisation: "",
    group: "",
    acceptedLegalNotice: false,
    groupSize: 0,
  });
}

export default new VerplantStore();
