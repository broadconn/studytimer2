import type { TimeSpan } from "../utils/time-span";

export class StudyItem {
  constructor(
    public name: string,
    public timeSpan: TimeSpan
  ) { }
}
