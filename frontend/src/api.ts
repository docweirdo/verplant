import moment from "moment";

export interface Course {
  id: number;
  name: string;
  default_duration?: number;
  default_room_id?: number;
  course_type?: string;
}

export enum AppointmentStatus {
  /// suggested to me
  Suggested = "SUGGESTED", 
  Approved = "APPROVED", 
  Rejected = "REJECTED", 
  /// suggested by me
  Pending = "PENDING"
}

export interface Appointment {
  id: number;
  date: moment.Moment;
  starttime: moment.Moment;
  endtime: moment.Moment;
  status: AppointmentStatus;
  proposer_id: number;
}

interface Api {
  getCourses(): Promise<Course[]>;
  getAppointments(bookingId: number): Promise<Appointment[]>; 
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
    return Promise.resolve([
      {
        id: 1,
        date: moment("12-08-1999", "DD-MM-YYYY"),
        starttime: moment("14:40", "hh:mm"),
        endtime: moment("15:40", "hh:mm"),
        status: AppointmentStatus.Rejected,
        proposer_id: 1
      },
      {
        id: 1,
        date: moment("15-08-1999", "DD-MM-YYYY"),
        starttime: moment("12:00", "hh:mm"),
        endtime: moment("13:00", "hh:mm"),
        status: AppointmentStatus.Pending,
        proposer_id: 1
      },
      {
        id: 1,
        date: moment("11-08-1999", "DD-MM-YYYY"),
        starttime: moment("16:00", "hh:mm"),
        endtime: moment("17:00", "hh:mm"),
        status: AppointmentStatus.Suggested,
        proposer_id: 2
      },
      {
        id: 1,
        date: moment("20-08-1999", "DD-MM-YYYY"),
        starttime: moment("10:20", "hh:mm"),
        endtime: moment("11:40", "hh:mm"),
        status: AppointmentStatus.Approved,
        proposer_id: 2
      }
    ]);
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
