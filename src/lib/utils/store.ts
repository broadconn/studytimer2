import { writable } from 'svelte/store';

export const studyPauseEventStore = writable(false);


export const uiChangeEventStore = writable({ isStudying: false, isShowingStudyListQueue: false, isShowingProgrammes: false });

export class UIChangeEventWriter {
  static setStudying(isStudying: boolean) {
    uiChangeEventStore.update((state) => {
      state.isStudying = isStudying;
      return state;
    });
  }

  static setShowingStudyListQueue(isShowingStudyListQueue: boolean) {
    uiChangeEventStore.update((state) => {
      state.isShowingStudyListQueue = isShowingStudyListQueue;
      return state;
    });
  }

  static setShowingProgrammes(isShowingProgrammes: boolean) {
    uiChangeEventStore.update((state) => {
      state.isShowingProgrammes = isShowingProgrammes;
      return state;
    });
  }
}
