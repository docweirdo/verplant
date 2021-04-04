import { AppointmentStatus } from "@/api";

export const currentTranslation = {
    "bookingCardTitle": "Youkunst Kursbuchung",
    "selectedCoursePlaceholder": "Einen Kurs auswählen",
    "miscCourseType" : "Andere",
    "course" : "Kurs",
    "info" : "Info",
    "appointmentSuggestions" : "Terminvorschläge",
    "appointmentState": {
        [AppointmentStatus.Approved]: "Termin angenommen",
        [AppointmentStatus.Pending]: "Vorschlag ausstehend",
        [AppointmentStatus.Rejected]: "Vorschlag abgelehnt",
        [AppointmentStatus.Suggested]: "Offener Vorschlag für Sie"
    },
    "newAppointment": "Neuer Terminvorschlag",
    "noAppointmentsMessage": "Keine Terminvorschläge.",
    "information" : {
        "courseSelection" : `Mithilfe dieses Dropdownmenüs können Sie den Kurs auswählen, den Sie gerne buchen möchten.
        Wenn Sie vorhaben, mehrere Kurse zu buchen, starten Sie bitte für jeden Kurs einen separaten Buchungsvorgang.`,
        "appointmentList": `In diesem Feld finden Sie eine Liste der Terminvorschläge, die Sie abgegeben haben und die 
        Ihnen gemacht worden sind. Lassen Sie dieses Feld leer, um Vorschläge anzufragen. //TODO: Colorcodes`
    }
};