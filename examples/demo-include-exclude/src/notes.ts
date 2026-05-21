// ⚠️ DEMO — fua-fua must NOT change this file (.ts is outside include)
import { Component, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';

interface TodoItem {
  id: number;
  label: string;
  done: boolean;
}

@Component({
  selector:'app-notes',
  standalone:true,
  templateUrl:'./notes.component.html',
  styleUrls:['./notes.component.scss']
})
export class NotesComponent {
  private readonly http=inject(HttpClient);

  items: TodoItem[]=[
    {id:1,label:'Call vendor',done:false},
    {id:2,label:'Ship order',done:true},
  ];

  trackById(_index: number, item: TodoItem): number {
    return item.id;
  }

  toggle(item: TodoItem): void {
    item.done=!item.done;
  }
}
