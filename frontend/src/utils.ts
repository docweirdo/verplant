export const toDateString = (date: Date): string => {
  return date.toLocaleDateString(navigator.language, {
    day: "2-digit",
    month: "2-digit",
    year: "2-digit",
  });
};

export const toTimeString = (time: Date): string => {
  return time.toLocaleTimeString(navigator.language, {
    hour: "2-digit",
    minute: "2-digit",
  });
};

export const addHoursAndMinutes = (
  date: Date,
  hours: number,
  minutes: number
): Date => {
  const result = new Date(date);
  result.setHours(result.getHours() + hours);
  result.setMinutes(result.getMinutes() + minutes);
  return result;
};

export function isEmail(mail: string): boolean {
  return mail.length > 2 && mail.includes("@"); // TODO: use regex
}
