// @ts-nocheck
import {Injectable, inject} from '@angular/core';
import {BehaviorSubject, Observable, of} from 'rxjs';
import {map, switchMap, tap} from 'rxjs/operators';
import {Playlist} from '../models/playlist';
import {AudioCollection} from '../models/audio-collection';
import {ColorUtility} from '../utils/color-utility';
import {HttpOptions, HttpService} from './http.service';
import {UserService} from './user.service';

@Injectable(
    {
    providedIn: 'root',
    }
)
export class PlaylistsService {
    private _base: string = 'playlist';
    
    private _playlists = new BehaviorSubject<Playlist[]>(
    []
    );
    public playlists$ = this._playlists.asObservable();
    private _httpService: HttpService = inject(HttpService);
    private _userService: UserService = inject(UserService);
    
    private _httpOptions: HttpOptions = {requiresAuthentication: true};
    
    public getUserPlaylists(): Observable<Playlist[]> {
    return this._userService.user$.pipe(
    switchMap(
    (user) => {
    if (!user) {
    this._playlists.next(
    []
    );
    return of(
    []
    );
    }
    
    const requestUrl = `${this._base}/user?userId=${user.id}&pageSize=100&pageNumber=1`;
    
    //const requestUrl = `http: //localhost: 80/api/playlist/user?userId=${this._userId}&pageSize=12&pageNumber=1`
    
    return this._httpService.get<Playlist[]>(requestUrl, this._httpOptions).pipe(
    map(
    (
    playlists: Playlist[]
    ) => {
    //Check whether playlists are null, if yes, set playlists to [] to avoid errors.
    if (playlists === null) {
    this._playlists.next(
    []
    );
    return [];
    } else {
    const updatedPlaylists = playlists.map(
    (playlist) => {
    return {
    ...playlist,
    collectionList: playlist.collectionList || [],
    tags: playlist.tags || [], // Ensure tags are not null.
    imageLoading: false,
    audioCollectionList: playlist.audioCollectionList || [],
    hiddenTags: playlist.hiddenTags || [],
    };
    }
    );
    this._playlists.next(updatedPlaylists);
    return updatedPlaylists;
    }
    }
    ),
    );
    }
    )
    );
    }
    
    public getPlaylistById(playlistId: string): Observable<Playlist> {
    const requestUrl = `${this._base}?playlistId=${playlistId}`;
    
    return this._httpService.get<Playlist>(requestUrl, this._httpOptions).pipe(
    map(
    (playlist: Playlist) => {
    return playlist;
    }
    ),
    tap(
    (playlist: Playlist) => {
    playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
    playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
    playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
    playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
    }
    ),
    );
    }
    
    public getPlaylistCollections(playlistId: string): Observable<AudioCollection[]> {
    const requestUrl = `${this._base}/allCollections?playlistId=${playlistId}`;
    
    return this._httpService.get<{
    collections: AudioCollection[]
    }>(requestUrl, this._httpOptions).pipe(
    map(
    (audioCollections) => {
    // Check if the response or the collections in the response are null.
    if (!audioCollections || !audioCollections.collections) {
    return [];
    }
    
    return audioCollections.collections.map(
    (collection) => {
    return {
    ...collection,
    color: ColorUtility.getRandomColor(),
    };
    }
    );
    }
    ),
    );
    }
    
    public updatePlaylist(
    playlistId: string, name?: string, collectionList?: string[], description?: string, tags?: string[]
    ): Observable<Playlist> {
    const body: UpdatePlaylistBody = {id: playlistId};
    
    if (name) {
    body.name = name;
    }
    
    if (collectionList) {
    body.collectionList = collectionList;
    }
    
    if (description) {
    body.description = description;
    }
    
    if (tags) {
    body.tags = tags;
    }
    
    const requestUrl = `${this._base}/update`;
    
    return this._httpService.patch<UpdatePlaylistBody, Playlist>(requestUrl, body, this._httpOptions).pipe(
    map(
    (playlist) => {
    const updatedPlaylist: Playlist = playlist;
    
    const newPlaylists: Playlist[] = [...this._playlists.value];
    newPlaylists.splice(
    newPlaylists.findIndex(
    (item: Playlist) => {
    return item.id === updatedPlaylist.id;
    }
    ),
    1,
    updatedPlaylist,
    );
    
    this._playlists.next(newPlaylists);
    
    return playlist;
    }
    ),
    tap(
    (playlist: Playlist) => {
    playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
    playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
    playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
    playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
    }
    ),
    );
    }
    
    public deletePlaylistById(playlistId: string): Observable<void> {
    const requestUrl = `${this._base}?playlistId=${playlistId}`;
    return this._httpService.delete<void>(requestUrl, this._httpOptions).pipe(
    tap(
    () => {
    this._playlists.next(
    this._playlists.value.filter(
    (playlist) => {
    return playlist.id !== playlistId;
    }
    ),
    );
    }
    ),
    );
    }
    
    public createPlaylist(name: string, description: string): Observable<Playlist> {
    const body: {
    name: string; collectionList: []; description: string
    } = {
    name: name, collectionList: [], description: description
    };
    
    const requestUrl = `${this._base}`;
    
    return this._httpService.post<{
    name: string; collectionList: []; description: string
    }, Playlist>(requestUrl, body, this._httpOptions).pipe(
    map(
    (playlist: Playlist) => {
    this._playlists.next(
    [...this._playlists.value, playlist]
    );
    return playlist;
    }
    ),
    tap(
    (playlist: Playlist) => {
    playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
    playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
    playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
    playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
    }
    ),
    );
    }
    
    public setPlaylistImage(playlistId: string, image: File): Observable<Playlist> {
    const formData = new FormData();
    formData.append('image', image);
    formData.append('playlistId', playlistId);
    
    const requestUrl = `${this._base}/setImage`;
    
    return this._httpService.put<FormData, Playlist>(requestUrl, formData, this._httpOptions).pipe(
    map(
    (playlist) => {
    playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
    playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
    playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
    return playlist;
    }
    ),
    );
    }
    
    public getImage(imageId: string): Observable<Uint8Array> {
    const requestUrl = `${this._base}/image?id=${imageId}`;
    return this._httpService.get<ArrayBuffer>(
    requestUrl, {responseType: 'arraybuffer'}
    ).pipe(
    map(
    (response: ArrayBuffer) => {
    return new Uint8Array(response);
    },
    ),
    );
    }
}

export interface UpdatePlaylistBody {
    id: string;
    name?: string;
    collectionList?: string[];
    description?: string;
    tags?: string[];
}// @ts-nocheck
import {Injectable, inject} from '@angular/core';
import {BehaviorSubject, Observable, of} from 'rxjs';
import {map, switchMap, tap} from 'rxjs/operators';
import {Playlist} from '../models/playlist';
import {AudioCollection} from '../models/audio-collection';
import {ColorUtility} from '../utils/color-utility';
import {HttpOptions, HttpService} from './http.service';
import {UserService} from './user.service';

@Injectable({
	providedIn: 'root',
})
export class PlaylistsService {
	private _base: string = 'playlist';

	private _playlists = new BehaviorSubject<Playlist[]>([]);
	public playlists$ = this._playlists.asObservable();
	private _httpService: HttpService = inject(HttpService);
	private _userService: UserService = inject(UserService);

	private _httpOptions: HttpOptions = {requiresAuthentication: true};

	public getUserPlaylists(): Observable<Playlist[]> {

		return this._userService.user$.pipe(
			switchMap((user) => {

				if (!user) {
					this._playlists.next([]);
					return of([]);
				}

				const requestUrl = `${this._base}/user?userId=${user.id}&pageSize=100&pageNumber=1`;

				//const requestUrl = `http: //localhost: 80/api/playlist/user?userId=${this._userId}&pageSize=12&pageNumber=1`

				return this._httpService.get<Playlist[]>(requestUrl, this._httpOptions).pipe(
					map((playlists: Playlist[]) => {
						//Check whether playlists are null, if yes, set playlists to [] to avoid errors.
						if (playlists === null) {
							this._playlists.next([]);
							return [];
						} else {
							const updatedPlaylists = playlists.map((playlist) => {
								return {
									...playlist,
									collectionList: playlist.collectionList || [],
									tags: playlist.tags || [], // Ensure tags are not null.
									imageLoading: false,
									audioCollectionList: playlist.audioCollectionList || [],
									hiddenTags: playlist.hiddenTags || [],
								};
							});
							this._playlists.next(updatedPlaylists);
							return updatedPlaylists;
						}
					}),
				);
			})
		);


	}

	public getPlaylistById(playlistId: string): Observable<Playlist> {

		const requestUrl = `${this._base}?playlistId=${playlistId}`;

		return this._httpService.get<Playlist>(requestUrl, this._httpOptions).pipe(
			map((playlist: Playlist) => {
				return playlist;
			}),
			tap((playlist: Playlist) => {
				playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
				playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
				playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
				playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
			}),
		);
	}

	public getPlaylistCollections(playlistId: string): Observable<AudioCollection[]> {

		const requestUrl = `${this._base}/allCollections?playlistId=${playlistId}`;

		return this._httpService.get<{collections: AudioCollection[]}>(requestUrl, this._httpOptions).pipe(
			map((audioCollections) => {
				// Check if the response or the collections in the response are null.
				if (!audioCollections || !audioCollections.collections) {
					return [];
				}

				return audioCollections.collections.map((collection) => {
					return {
						...collection,
						color: ColorUtility.getRandomColor(),
					};
				});
			}),
		);
	}

	public updatePlaylist(playlistId: string, name?: string, collectionList?: string[], description?: string, tags?: string[]): Observable<Playlist> {
		const body: UpdatePlaylistBody = {id: playlistId};

		if (name) {
			body.name = name;
		}

		if (collectionList) {
			body.collectionList = collectionList;
		}

		if (description) {
			body.description = description;
		}

		if (tags) {
			body.tags = tags;
		}

		const requestUrl = `${this._base}/update`;

		return this._httpService.patch<UpdatePlaylistBody, Playlist>(requestUrl, body, this._httpOptions).pipe(
			map(
				(playlist) => {
					const updatedPlaylist: Playlist = playlist;

					const newPlaylists: Playlist[] = [...this._playlists.value];
					newPlaylists.splice(
						newPlaylists.findIndex(
								(item: Playlist) => {
							return item.id === updatedPlaylist.id;
						}),
						1,
						updatedPlaylist,
					);

					this._playlists.next(newPlaylists);

					return playlist;
				}
			),
			tap((playlist: Playlist) => {
				playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
				playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
				playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
				playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
			}),
		);
	}

	public deletePlaylistById(playlistId: string): Observable<void> {
		const requestUrl = `${this._base}?playlistId=${playlistId}`;
		return this._httpService.delete<void>(requestUrl, this._httpOptions).pipe(
			tap(() => {
				this._playlists.next(
					this._playlists.value.filter((playlist) => {
						return playlist.id !== playlistId;
					}),
				);
			}),
		);
	}

	public createPlaylist(name: string, description: string): Observable<Playlist> {
		const body: {name: string; collectionList: []; description: string} = {name: name, collectionList: [], description: description};

		const requestUrl = `${this._base}`;

		return this._httpService.post<{name: string; collectionList: []; description: string}, Playlist>(requestUrl, body, this._httpOptions).pipe(
			map((playlist: Playlist) => {
				this._playlists.next([...this._playlists.value, playlist]);
				return playlist;
			}),
			tap((playlist: Playlist) => {
				playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
				playlist.hiddenTags = playlist.hiddenTags || []; // Ensure hiddenTags are not null.
				playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
				playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
			}),
		);
	}

	public setPlaylistImage(playlistId: string, image: File): Observable<Playlist> {
		const formData = new FormData();
		formData.append('image', image);
		formData.append('playlistId', playlistId);

		const requestUrl = `${this._base}/setImage`;

		return this._httpService.put<FormData, Playlist>(requestUrl, formData, this._httpOptions).pipe(
			map((playlist) => {
				playlist.collectionList = playlist.collectionList || []; // Ensure collectionList is not null.
				playlist.tags = playlist.tags || []; // Ensure collectionList is not null.
				playlist.audioCollectionList = []; // Set list of audio collection objects - empty array.
				return playlist;
			}),
		);
	}

	public getImage(imageId: string): Observable<Uint8Array> {
		const requestUrl = `${this._base}/image?id=${imageId}`;
		return this._httpService.get<ArrayBuffer>(requestUrl, {responseType: 'arraybuffer'}).pipe(
			map(
				(response: ArrayBuffer) => {
					return new Uint8Array(response);
				},
			),
		);
	}
}

export interface UpdatePlaylistBody {
	id: string;
	name?: string;
	collectionList?: string[];
	description?: string;
	tags?: string[];
}