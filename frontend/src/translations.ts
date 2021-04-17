import { AppointmentStatus } from "@/api";

export const currentTranslation = {
  bookingCardTitle: "YouKunst Kursbuchung",
  selectedCoursePlaceholder: "Einen Kurs auswählen",
  miscCourseType: "Andere",
  course: "Kurs",
  info: "Info",
  firstname: "Vorname",
  lastname: "Nachname",
  email: "Email",
  phone: "Telefon",
  organisation: "Organisation",
  class: "Klasse / Kurs",
  password: "Passwort",
  login: "Einloggen",
  loginError: "Email oder Passwort sind falsch",
  loginCardTitle: "YouKunst Login",
  appointmentSuggestions: "Terminvorschläge",
  appointmentState: {
    [AppointmentStatus.Approved]: "Termin angenommen",
    [AppointmentStatus.Pending]: "Antwort ausstehend",
    [AppointmentStatus.Rejected]: "Vorschlag abgelehnt",
    [AppointmentStatus.Suggested]: "Offener Vorschlag an Sie",
  },
  newAppointment: "Neuer Terminvorschlag",
  noAppointmentsMessage: "Keine Terminvorschläge.",
  add: "Hinzufügen",
  next: "Weiter",
  send: "Abschicken",
  back: "Zurück",
  date: "Datum",
  startTime: "Startzeit",
  endTime: "Ende",
  legalNotice: "Ich habe die AGB und Datenschutzbestimmungen akzeptiert", 
  information: {
    courseSelection: `Mithilfe dieses Dropdownmenüs können Sie den Kurs auswählen, den Sie gerne buchen möchten.
        Wenn Sie vorhaben, mehrere Kurse zu buchen, starten Sie bitte für jeden Kurs einen separaten Buchungsvorgang.`,
    appointmentList: `In diesem Feld finden Sie eine Liste der Terminvorschläge, die Sie abgegeben haben und die 
        Ihnen gemacht worden sind. Lassen Sie dieses Feld leer, um Vorschläge anzufragen. //TODO: Colorcodes`,
  },
};
