import moment from "moment";

export interface Course {
  id: number;
  name: string;
  default_duration?: number;
  default_room_id?: number;
  course_type?: string;
}

export interface Appointment {
  id: number;
  date: moment.Moment;
  starttime: moment.Moment;
  endtime: moment.Moment;
  status: string;
  proposer_id: number;
}

interface Api {
  getCourses(): Promise<Course[]>;
  getAppointments(bookingId: number): Promise<Appointment[]>; // TODO: type
}

class FakeApi implements Api {
  getCourses() {
    return Promise.resolve([
      {
        id: 1,
        name: "Photoshop Kreativkurs Edition 2000",
        course_type: "Digital",
      },
      {
        id: 2,
        name: "CAD",
        course_type: "Digital",
      },
      {
        id: 3,
        name: "Aktzeichnen",
        course_type: "Zeichnen",
      },
      {
        id: 4,
        name: "Kartoffeldruck",
        course_type: "Zeichnen",
      },
    ]);
  }

  getAppointments(bookingId: number): Promise<Appointment[]> {
    return Promise.resolve([]);
  }
}

class HttpApi implements Api {
  async getCourses(): Promise<Course[]> {
    const result = await fetch("/api/courses", { credentials: "include" });
    const obj = await result.json();
    if (!Array.isArray(obj)) {
      console.warn("Expected array");
    }
    return obj;
  }

  async getAppointments(bookingId : number): Promise<Appointment[]> {
    const result = await fetch(`/api/bookings/${bookingId}`, { credentials: "include" });
    const obj = await result.json() as any[];
    if (!Array.isArray(obj)) {
      console.warn("Expected array");
    }
    return obj.map(appointment => {
      return {
        ...appointment,
        date: moment(appointment.date, "DD-MM-YYYY"),
        starttime: moment(appointment.starttime, "hh:mm"),
        endtime: moment(appointment.endtime, "hh:mm"),
      };
    });
  }
}

export const api: Api = new FakeApi();
//export const api: Api = new HttpApi();
