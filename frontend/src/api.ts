import store from '@/store'


export interface Course {
  id: number;
  name: string;
  default_duration?: number;
  default_room_id?: number;
  course_type?: string;
}

/**
 * Describes a single appointment date and the time range
 */
export interface AppointmentSuggestion {
  from: Date;
  to: Date;
}

export enum AppointmentStatus {
  /// suggested to me
  Suggested = "SUGGESTED",
  Approved = "APPROVED",
  Rejected = "REJECTED",
  /// suggested by me
  Pending = "PENDING",
}

export interface Appointment {
  id?: number;
  starttime: Date;
  endtime: Date;
  status: AppointmentStatus;
  proposer_id?: number;
}

interface Api {
  getCourses(): Promise<Course[]>;
  getAppointments(bookingURL: string): Promise<Appointment[]>;
  /**
   * returns true if successfull, false if credentials were wrong, throws on response error
   */
  login(email: string, password: string): Promise<boolean>;
  updateAppointments(bookingUrl: string, updated: Appointment[]): Promise<void>;
  addAppointments(bookingUrl: string, added: Appointment[]): Promise<void>;
  withdrawAppointments(bookingUrl: string, withdrawn: Appointment[]): Promise<void>;
  /**
   * returns the new URL
   */
  createNewBooking(customerInfos: typeof store.contactInformations): Promise<string>;
}

class FakeApi implements Api {
  private loggedIn = false;

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

  async login(email: string, password: string): Promise<boolean> {
    console.log("fakeApi login", email, password);
    if (email == "test@test.de") {
      this.loggedIn = true;
      return true;
    }
    return false;
  }

  getAppointments(bookingURL: string): Promise<Appointment[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([
          {
            id: 1,
            starttime: new Date("2021-02-01T14:32Z"),
            endtime: new Date("2021-02-01T15:32Z"),
            status: AppointmentStatus.Rejected,
            proposer_id: 1,
          },
          {
            id: 2,
            starttime: new Date("2021-05-01T14:32Z"),
            endtime: new Date("2021-05-01T14:12Z"),
            status: AppointmentStatus.Pending,
            proposer_id: 1,
          },
          {
            id: 3,
            starttime: new Date("2021-05-02T14:32Z"),
            endtime: new Date("2021-05-02T14:12Z"),
            status: AppointmentStatus.Suggested,
            proposer_id: 2,
          },
          {
            id: 4,
            starttime: new Date("2021-05-04T14:32Z"),
            endtime: new Date("2021-05-04T14:12Z"),
            status: AppointmentStatus.Approved,
            proposer_id: 2,
          },
          {
            id: 5,
            starttime: new Date("2021-05-01T14:32Z"),
            endtime: new Date("2021-05-01T14:12Z"),
            status: AppointmentStatus.Pending,
            proposer_id: 1,
          },
          {
            id: 6,
            starttime: new Date("2021-05-02T14:32Z"),
            endtime: new Date("2021-05-02T14:12Z"),
            status: AppointmentStatus.Suggested,
            proposer_id: 2,
          },
          {
            id: 7,
            starttime: new Date("2021-05-04T14:32Z"),
            endtime: new Date("2021-05-04T14:12Z"),
            status: AppointmentStatus.Approved,
            proposer_id: 2,
          },
        ]);
      }, 0);
    });

  }

  async updateAppointments(bookingURL : string, updated : Appointment[]): Promise<void> {
    console.log('updated appointments', updated);
    return Promise.resolve();
  }

  async addAppointments(bookingURL : string, added : Appointment[]): Promise<void> {
    console.log('added appointments', added);
    
    return Promise.resolve();
  }

  async withdrawAppointments(bookingURL : string, withdrawn : Appointment[]): Promise<void> {
    console.log('withdrawn appointments', withdrawn);
    
    return Promise.resolve();
  }

  createNewBooking(customerInfos: typeof store.contactInformations): Promise<string> {
    console.log('createNewBooking', customerInfos);
    return Promise.resolve('fake-url');
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

  async login(email: string, password: string): Promise<boolean> {
    const result: Response = await fetch("/api/auth/login", {
      method: "POST",
      body: JSON.stringify({
        email: email,
        password: password,
      }),
    });

    return result.status == 200;
  }

  async getAppointments(bookingURL: string): Promise<Appointment[]> {
    const result = await fetch(`/api/bookings/${bookingURL}/appointments`, {
      credentials: "include",
    });
    const obj = (await result.json()) as any[];
    if (!Array.isArray(obj)) {
      console.warn("Expected array");
    }
    return obj.map((appointment) => {
      return {
        ...appointment,
        starttime: new Date(appointment.starttime),
        endtime: new Date(appointment.endtime),
      } as Appointment;
    });
  }

  async updateAppointments(bookingURL : string, updated : Appointment[]): Promise<void> {
   
    return Promise.resolve();
  }

  async addAppointments(bookingURL : string, added : Appointment[]): Promise<void> {
    console.log('added appointments', added);
    
    return Promise.resolve();
  }

  async withdrawAppointments(bookingURL : string, withdrawn : Appointment[]): Promise<void> {
    console.log('withdrawn appointments', withdrawn);
    
    return Promise.resolve();
  }

  async createNewBooking(args: typeof store.contactInformations): Promise<string> {
    return Promise.resolve("safgeasg")
  }
}

export const api: Api = new FakeApi();
//export const api: Api = new HttpApi();
