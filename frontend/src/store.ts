import { reactive } from "@vue/reactivity";

class VerplantStore {
    contactInformations = reactive({
        firstname: "",
        lastname: "",
        email: "",
        phone: "",
        organisation: "",
        group: "",
        acceptedLegalNotice: false
    });


}

export default new VerplantStore();