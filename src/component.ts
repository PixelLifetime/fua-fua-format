import { Component, ElementRef, inject, OnInit, PLATFORM_ID, ViewChild } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { AudioCollection } from '../../models/audio-collection';
import { AudioCollectionsService, UpdateAudioCollectionBody, UpdateAudioListBody } from '../../api/audio-collections.service';
import { AudioService } from '../../api/audio.service';
import { Audio, AudioMetadata, CombinedAudioFile } from '../../models/audio-file';
import { ScrollViewComponent } from '../../../../projects/pixli-angular/src/lib/scroll-view/scroll-view.component';
import { AudioFileCardComponent } from '../../components/audioFiles/audio-file-card/audio-file-card.component';
import { FormsModule, NgForm } from '@angular/forms';
import { CommonModule, isPlatformBrowser } from '@angular/common';
import { DropZoneDirective } from '../../directives/drop-zone.directive';
import { AudioFileCardSkeletonComponent } from '../../components/skeletons/audio-file-card-skeleton/audio-file-card-skeleton.component';
import { WaveSurfer } from 'wavesurfer.js';
import { WavesurferUtils } from '../../utils/wavesurfer.utils';
import { ProgressbarComponent } from '../../components/progressbar/progressbar.component';
import { v4 as uuidv4 } from 'uuid';
import { Point, Tangent } from '../../components/checkered-canvas/checkered-canvas.component';

@Component(
    {
        selector: 'app-collection',
        standalone: true,
        templateUrl: './collection.component.html',
        styleUrl: './collection.component.scss',
        providers: [WavesurferUtils],
        imports: [
            FormsModule, ScrollViewComponent, AudioFileCardComponent, CommonModule, DropZoneDirective, AudioFileCardSkeletonComponent, ProgressbarComponent
        ],
    }
)
export class CollectionComponent implements OnInit {
    private _collectionId!: string;

    public audioCollection!: AudioCollection;

    public editableAudioCollection!: AudioCollection;

    public ambienceReferenceAudioCollection!: AudioCollection;

    private _route = inject(ActivatedRoute);
    private _audioCollectionsService = inject(AudioCollectionsService);
    private _audioService = inject(AudioService);

    public searchedAudioFiles?: Audio[];

    public isLoadingSearchedAudioCollections: boolean = false;

    public repeatSkeleton: number[] = new Array(5).fill(0);

    public imageUrl: string = '';

    private _editingMode: boolean = false;
    public get editingMode(): boolean {
        return this._editingMode;
    }
    public set editingMode(value: boolean) {
        //If we set editing mode to true - playing of the ambience should be ended
        if (value) {
            if (this.waveSurfer) {
                this.isAmbiencePlaying = false;
                this.waveSurfer.stop();
            }
        }
        this._editingMode = value;
    }

    private imageToSet?: File;

    @ViewChild('form', {static: true})
    public form!: NgForm;

    public nameUpdateField: string = '';

    public descriptionUpdateField: string = '';

    private platformId = inject(PLATFORM_ID);

    @ViewChild('waveform')
    private _waveformContainer!: ElementRef<HTMLElement>;

    private _waveSurfer!: WaveSurfer;

    public get waveSurfer(): WaveSurfer {
        return this._waveSurfer;
    }
    public generatedAmibienceUrl!: string;

    private _wavesurferUtils: WavesurferUtils = inject(WavesurferUtils);

    ngOnInit(): void {
        if (
            isPlatformBrowser(this.platformId)
        ) {
            this._audioCtx = new AudioContext();
        }

        this._route.queryParams.subscribe(
            (params) => {
                this._collectionId = params['id'];
                this._audioCollectionsService.getAudioCollectionById(this._collectionId).subscribe(
                    {
                        next: (audioCollection) => {
                            this._getImage(audioCollection.imageId);
                            this.audioCollection = audioCollection;
                            this.editableAudioCollection ={...audioCollection};
                            this.ambienceReferenceAudioCollection = {...audioCollection};
                            console.log(audioCollection);
                            this._getAudioCollectionAudioObjects();
                            this.nameUpdateField = audioCollection.name;
                            this.descriptionUpdateField = audioCollection.description;
                            this.editingMode = false;
                        },
                    }
                );
            }
        );

        //To display some audios in the search element.
        this.initiateSearch();
    }

    private _getImage(imageId: string): void {
        if (imageId !== '00000000-0000-0000-0000-000000000000') {
            this._audioCollectionsService.getImage(imageId).subscribe(
                {
                    next: (image: Uint8Array) => {
                        const blob = new Blob([image]);
                        const url = URL.createObjectURL(blob);
                        this.imageUrl = url;
                    },
                }
            );
        }
    }

    private _getAudioCollectionAudioObjects(): void {
        if (this.audioCollection.audioList.length !== 0) {
            this._audioCollectionsService.getAudioCollectionAudios(this._collectionId).subscribe(
                {
                    next: (audios) => {
                        this.audioCollection.audioList = audios;
                        this.editableAudioCollection.audioList = audios;
                    },
                    error: (error) => {
                        console.log(error);
                    },
                }
            );
        }
    }

    public searchQuery: string = '';

    public initiateSearch(): void {
        this.isLoadingSearchedAudioCollections = true;
        this._audioService.search(this.searchQuery).subscribe(
            {
                next: (audioFiles) => {
                    this.searchedAudioFiles = [];
                    console.log(audioFiles);

                    if (audioFiles) {
                        audioFiles.forEach(
                            (audioFile) => {
                                this.searchedAudioFiles!.push(
                                    new Audio(
                                        audioFile.id, {
                                            audioId: audioFile.id, playbackOffset: 0, volume: 0.5, id: undefined, splinePoints: []
                                        }, audioFile
                                    )
                                );
                            }
                        );
                    }
                },
            }
        );
        this.isLoadingSearchedAudioCollections = false;
    }

    public getImageUrl(): string {
        return `url(${this.imageUrl})`;
    }

    public handleImageFileUpload(event: Event): void {
        const inputElement = event.target as HTMLInputElement;
        if (inputElement.files) {
            this.imageToSet = inputElement.files[0];
        }
    }

    public onImageFileDropped(files: File[] | Event): void {
        const fileArray = files instanceof Array ? files: [];
        this.imageToSet = fileArray[0];
    }

    public setImage(): void {
        this._audioCollectionsService.setAudioCollectionImage(this.audioCollection.id, this.imageToSet!).subscribe(
            {
                next: (audioCollection) => {
                    this._getImage(audioCollection.imageId);
                },
            }
        );
    }

    public addAudioToCollection(audioFileIdToAdd: string): void {
        // console.log(audioFileIdToAdd);
        // if (
        // 	!this.editableAudioCollection.audioList.some((audio) => {
        // 		return audio.audioId === audioFileIdToAdd;
        // 	})
        // ) {
        // 	this._audioService.getAudioFileById(audioFileIdToAdd).subscribe({
        // 		next: (audioFile) => {
        // 			this.editableAudioCollection.audioList = [...this.editableAudioCollection.audioList, new Audio(new AudioMetadata(0.5, 0), audioFile, audioFileIdToAdd)];
        // 		},
        // 	});
        // }
        this._audioService.getAudioFileById(audioFileIdToAdd).subscribe(
            {
                next: (audioFile) => {
                    //this.editableAudioCollection.audioList = [...this.editableAudioCollection.audioList, new Audio(new AudioMetadata(0.5, 0), audioFile, audioFileIdToAdd)];
                    const temporaryId = uuidv4();
                    this.editableAudioCollection.audioList = [
                        ...this.editableAudioCollection.audioList, new Audio(
                            audioFileIdToAdd, new AudioMetadata(
                                audioFileIdToAdd, 0.5, 0, [
                                    new Point(
                                        0, 1, new Tangent(0, 0), new Tangent(0, 0), false
                                    ), new Point(
                                        audioFile.duration / 1000, 1, new Tangent(0, 0), new Tangent(0, 0), false
                                    )
                                ], temporaryId
                            ), audioFile
                        )
                    ];
                },
            }
        );
    }

    public deleteAudioFromCollection(audioFileIdToDelete: string): void {
        console.log(this.editableAudioCollection.audioList);
        this.editableAudioCollection.audioList = this.editableAudioCollection.audioList.filter(
            (audioFile) => {
                return audioFile.audioMetadata.id !== audioFileIdToDelete;
            }
        );
    }

    private _getAudioCollectionUpdateParams(): UpdateAudioCollectionBody {
        this.editableAudioCollection.name = this.nameUpdateField;
        this.editableAudioCollection.description = this.descriptionUpdateField;
        const audioCollectionUpdateParams: UpdateAudioCollectionBody ={id: this._collectionId};

        if (this.audioCollection.name !== this.editableAudioCollection.name) {
            audioCollectionUpdateParams.name = this.editableAudioCollection.name;
        }

        if (this.audioCollection.audioList !== this.editableAudioCollection.audioList) {
            const audioList: UpdateAudioListBody[] = [];

            this.editableAudioCollection.audioList.forEach(
                (audio) => {
                    audioList.push(
                        {
                            audioId: audio.audio ? audio.audio.id: audio.audioId,
                            playbackOffset: audio.audioMetadata.playbackOffset,
                            volume: audio.audioMetadata.volume,
                            splinePoints: audio.audioMetadata.splinePoints,
                        }
                    );
                }
            );

            audioCollectionUpdateParams.audioList = audioList;
        }

        if (this.audioCollection.description !== this.editableAudioCollection.description) {
            audioCollectionUpdateParams.description = this.editableAudioCollection.description;
        }

        if (this.audioCollection.tags !== this.editableAudioCollection.tags) {
            audioCollectionUpdateParams.tags = this.editableAudioCollection.tags;
        }

        return audioCollectionUpdateParams;
    }

    public handleUpdateAudioCollection(confirmed: boolean): void {
        if (confirmed) {
            const audioCollectionUpdateParams: UpdateAudioCollectionBody = this._getAudioCollectionUpdateParams();

            if (this.imageToSet) {
                this.setImage();
            }

            if (audioCollectionUpdateParams.name === undefined && audioCollectionUpdateParams.audioList === undefined && audioCollectionUpdateParams.description === undefined && audioCollectionUpdateParams.tags === undefined) {
                console.log('nothing to update there');
            } else {
                this._audioCollectionsService.updateAudioCollection(
                    audioCollectionUpdateParams.id, audioCollectionUpdateParams.name, audioCollectionUpdateParams.audioList, audioCollectionUpdateParams.description, audioCollectionUpdateParams.tags
                ).subscribe(
                    {
                        next: (updatedAudioCollection) => {
                            this.audioCollection = updatedAudioCollection;
                            this.editableAudioCollection ={...updatedAudioCollection};
                            this._getAudioCollectionAudioObjects();
                            this.nameUpdateField = this.audioCollection.name;
                            this.descriptionUpdateField = this.audioCollection.description;
                        },
                    }
                );
            }

            this.editingMode = false;
        }
    }

    public handleAudioVolumeChange(updatedAudio: Audio): void {
        if (this.editingMode) {
            this.editableAudioCollection.audioList = this.editableAudioCollection.audioList.map(
                (audio) => {
                    if (audio.audioMetadata.id === updatedAudio.audioMetadata.id) {
                        return updatedAudio;
                    } else {
                        return audio;
                    }
                }
            );
        } else {
            this.audioCollection.audioList = this.audioCollection.audioList.map(
                (audio) => {
                    if (audio.audioMetadata.id === updatedAudio.audioMetadata.id) {
                        return updatedAudio;
                    } else {
                        return audio;
                    }
                }
            );
        }
    }

    public handleAudioPlaybackOffsetChange(updatedAudio: Audio): void {
        if (this._editingMode) {
            this.editableAudioCollection.audioList = this.editableAudioCollection.audioList.map(
                (audio) => {
                    if (audio.audioMetadata.id === updatedAudio.audioMetadata.id) {
                        return updatedAudio;
                    } else {
                        return audio;
                    }
                }
            );
        } else {
            this.audioCollection.audioList = this.audioCollection.audioList.map(
                (audio) => {
                    if (audio.audioMetadata.id === updatedAudio.audioMetadata.id) {
                        return updatedAudio;
                    } else {
                        return audio;
                    }
                }
            );
        }
    }

    // The AudioContext is where we will do our audio processing
    private _audioCtx!: AudioContext;

    public isAmbiencePlaying: boolean = false;

    public isLoadingAmbience: boolean = false;

    public ambienceLoadingProgress: number = 0;

    public ambiencePlayingProgress!: number;

    public ambienceDuration!: number;

    private async _decodeAudioFiles(): Promise<CombinedAudioFile[]> {
        const files: CombinedAudioFile[] = [];

        // Uploading files from the server... (0)
        await Promise.all(
            this.audioCollection.audioList?.map(
                async (audioFile, index) => {
                    if (audioFile.audio?.fileBytes) {
                        console.warn('there are already fileBytes');

                        files.push(
                            new CombinedAudioFile(
                                audioFile.audioMetadata.playbackOffset, audioFile.audioMetadata.volume, audioFile.audioMetadata.splinePoints, audioFile.audio?.fileBytes
                            )
                        );
                    } else {
                        const fileBytes = await this._audioService.getAudio(audioFile.audio!.fileId).toPromise();
                        audioFile.audio!.fileBytes = fileBytes;

                        this.audioCollection.audioList[index] = audioFile;

                        files.push(
                            new CombinedAudioFile(
                                audioFile.audioMetadata.playbackOffset, audioFile.audioMetadata.volume, audioFile.audioMetadata.splinePoints, fileBytes
                            )
                        );
                    }
                }
            ),
        );

        // Converting audio files... (1)
        this.ambienceLoadingProgress = 1;

        /**
        * This code creates array with type ArrayBuffer,
        * then gets each of file (with type Uint8Array) in files array,
        * generates blob for it, and then converts it into ArrayBuffer
        */
        const buffers: CombinedAudioFile[] = await Promise.all(
            files.map(
                async (file) => {
                    const reader = new FileReader();

                    return new Promise(
                        (resolve, reject) => {
                            const blob = new Blob([file.rawBuffer!]);

                            reader.onload = () => {
                                return resolve(
                                    new CombinedAudioFile(
                                        file.playbackOffset, file.volume, file.splinePoints, undefined, reader.result as ArrayBuffer
                                    )
                                );
                            };
                            reader.onerror = reject;
                            reader.readAsArrayBuffer(blob);
                        }
                    );
                }
            ),
        );

        // Decoding audio files... (2)
        this.ambienceLoadingProgress = 2;

        return await Promise.all(
            buffers.map(
                async (buffer) => {
                    return new CombinedAudioFile(
                        buffer.playbackOffset, buffer.volume, buffer.splinePoints,  undefined, undefined, await this._audioCtx.decodeAudioData(buffer.predecodedBuffer!)
                    );
                },
            ),
        );
    }

    /**
    * Function to mix audio buffers
    */
    public async mixAudioBuffers(): Promise<void> {
        this.isLoadingAmbience = true;
        const buffers: CombinedAudioFile[] = await this._decodeAudioFiles();

        // Determine the longest buffer length
        const longestBufferLength = Math.max(
            ...buffers.map(
                (buffer) => {
                    return buffer.decodedBuffer!.length;
                }
            ),
        );

        const outputBuffer = this._audioCtx.createBuffer(2, longestBufferLength, this._audioCtx.sampleRate);

        // Combining audio files... (3)
        this.ambienceLoadingProgress = 3;

        buffers.forEach(
            (buffer) => {
                const decodedBuffer = buffer.decodedBuffer!;
                const bufferLength = decodedBuffer.length;
                const offsetSamples = (buffer.playbackOffset / 1000) * decodedBuffer.sampleRate;

                for (let channel = 0; channel < outputBuffer.numberOfChannels; channel++) {
                    const outputData = outputBuffer.getChannelData(channel);

                    if (channel < decodedBuffer.numberOfChannels) {
                        const inputData = decodedBuffer.getChannelData(channel);

                        for (let i = 0; i < outputBuffer.length; i++) {
                            const inputIndex = (i + offsetSamples) % bufferLength;

                            outputData[i] += inputData[inputIndex] * buffer.volume * this.interpolate(inputIndex / bufferLength, buffer.splinePoints).y;
                        }
                    }
                }
            }
        );

        // Applying audio files to wavesurfer ... (4)
        this.ambienceLoadingProgress = 4;

        await this._loadAudio(outputBuffer);

        // Ambience loading finished ... (5)
        this.ambienceLoadingProgress = 5;

        this.isLoadingAmbience = false;

        this.ambienceReferenceAudioCollection ={...this.audioCollection};
    }

    /**
    * Performs cubic Hermite interpolation between two points.
    * This method calculates an intermediate point between two control points based on the parameter t,
    * which ranges from 0 to 1, where 0 represents the first point and 1 represents the second point.
    *
    * @param t - The parameter ranging from 0 to 1 for interpolation.
    * @param p0 - The starting control point.
    * @param p1 - The ending control point.
    * @returns The interpolated point.
    */
    private _cubicHermiteInterpolate(t: number, p0: Point, p1: Point): Point {
        const t2 = t * t;
        const t3 = t2 * t;

        const h0 = 2 * t3 - 3 * t2 + 1;
        const h1 = -2 * t3 + 3 * t2;
        const h2 = t3 - 2 * t2 + t;
        const h3 = t3 - t2;

        const position: Point = {
            x: p0.x * h0 + p1.x * h1 + p0.rightTangent.x * h2 + p1.leftTangent.x * h3,
            y: p0.y * h0 + p1.y * h1 - p0.rightTangent.y * h2 - p1.leftTangent.y * h3,
            leftTangent: p0.leftTangent,
            rightTangent: p0.rightTangent,
        };

        return position;
    }


    /**
    * Interpolates a point on the spline based on a parameter t.
    * This method finds the appropriate segment of the spline to interpolate between and returns
    * the interpolated point.
    *
    * @param t - The parameter ranging from 0 to 1 to determine the position on the spline.
    * @returns The interpolated point on the spline.
    */
    public interpolate(t: number, splinePoints: Point[]): Point {
        const segmentLength = 1 / (splinePoints.length - 1);

        const segmentIndex = Math.min(
            Math.floor(t / segmentLength), splinePoints.length - 1
        );
        const localT = (t % segmentLength) / segmentLength;

        const p0 = splinePoints[segmentIndex];
        const p1 = splinePoints[segmentIndex + 1];

        const m0Left = splinePoints[segmentIndex].leftTangent;
        const m0Right = splinePoints[segmentIndex].rightTangent;

        let x = 0;
        let y = 0;

        if (t == 1) {
            x = splinePoints[segmentIndex].x;
            y = splinePoints[segmentIndex].y;
        } else {
            x = this._cubicHermiteInterpolate(localT, p0, p1).x;
            y = this._cubicHermiteInterpolate(localT, p0, p1).y;
        }

        return {
            x: x, y: y, leftTangent: m0Left, rightTangent: m0Right
        };
    }

    /**
    * Function to load audioBuffer into wavesurfer and generate custom waveform for that.
    */
    public async _loadAudio(buffer: AudioBuffer): Promise<void> {
        const convertedAudio = this._audioBufferToWav(buffer);

        const blob = new Blob([convertedAudio], {type: 'audio/wav'});

        const url = URL.createObjectURL(blob);

        this.generatedAmibienceUrl = url;

        if (this.waveSurfer) {
            this.waveSurfer.destroy();
        }

        await this._wavesurferUtils.waveformFromFile(url, this._waveformContainer).then(
            (waveSurfer) => {
                return (this._waveSurfer = waveSurfer);
            }
        );
    }

    /**
    * Function to convert audioBuffer into ArrayBuffer.
    */
    private _audioBufferToWav(buffer: AudioBuffer): ArrayBuffer {
        const numOfChan = buffer.numberOfChannels;
        const length = buffer.length * numOfChan * 2 + 44;
        const bufferArray = new ArrayBuffer(length);
        const view = new DataView(bufferArray);
        const channels = [];
        let i,
        sample,
        offset = 0,
        pos = 0;

        // write WAVE header
        setUint32(0x46464952); // "RIFF"
        setUint32(length - 8); // file length - 8
        setUint32(0x45564157); // "WAVE"

        setUint32(0x20746d66); // "fmt " chunk
        setUint32(16); // length = 16
        setUint16(1); // PCM (uncompressed)
        setUint16(numOfChan);
        setUint32(buffer.sampleRate);
        setUint32(buffer.sampleRate * 2 * numOfChan); // avg. bytes/sec
        setUint16(numOfChan * 2); // block-align
        setUint16(16); // 16-bit (hardcoded in this demo)

        setUint32(0x61746164); // "data" - chunk
        setUint32(length - pos - 4); // chunk length

        // write interleaved data
        for (i = 0; i < buffer.numberOfChannels; i++) {
            channels.push(
                buffer.getChannelData(i)
            );
        }

        while (pos < length) {
            for (i = 0; i < numOfChan; i++) {
                // interleave channels
                sample = Math.max(
                    -1, Math.min(1, channels[i][offset])
                ); // clamp
                sample = (sample < 0 ? sample * 0x8000: sample * 0x7fff) | 0; // scale to 16-bit signed int
                view.setInt16(pos, sample, true); // write 16-bit sample
                pos += 2;
            }
            offset++; // next source sample
        }

        function setUint16(data: number) {
            view.setUint16(pos, data, true);
            pos += 2;
        }

        function setUint32(data: number) {
            view.setUint32(pos, data, true);
            pos += 4;
        }

        return bufferArray;
    }

    /**
    * Function to check wether we have to regenerate ambience, due to metadata change, or
    */
    public async toggleAmbience(): Promise<void> {
        if (this.isAmbiencePlaying) {
            console.warn('stopping ambience');
            console.log(
                this.waveSurfer.getCurrentTime()
            );
            this.ambiencePlayingProgress = this.waveSurfer.getCurrentTime();
            this.ambienceDuration = this.waveSurfer.getDuration();
            this.waveSurfer.stop();
            this.waveSurfer.seekTo(this.ambiencePlayingProgress / this.ambienceDuration);
            this.isAmbiencePlaying = false;
        } else {
            console.warn('starting to play ambience');

            await this._waveSurfer.play();
            console.log(this.ambiencePlayingProgress);

            /**
            * When the audio finishes, play it from the beginning
            */
            this._waveSurfer.on(
                'finish', () => {
                    if (this.isAmbiencePlaying) {
                        this._waveSurfer.seekTo(0); // Seek to the beginning
                        this._waveSurfer.play(); // Start playback
                    }
                }
            );
            this.isAmbiencePlaying = true;
        }
    }

    public async generateAmbience(): Promise<void> {
        if (!this.isAmbiencePlaying) {
            console.warn('regenerating ambience');
            await this.mixAudioBuffers();
            this.incrementAudioCollectionPlays();
        } else {
            if (this.generatedAmibienceUrl) {
                console.warn('stopping ambience');
                this.waveSurfer.stop();
                this.isAmbiencePlaying = false;
                this.generateAmbience();
            }
        }
    }

    public incrementAudioCollectionPlays(): void {
        this._audioCollectionsService.incrementAudioCollectionPlays(this.audioCollection.id).subscribe(
            {
                next: () => {
                    // this.audioCollection.plays! = this.audioCollection.plays + 1;
                }
            }
        );
    }

    /**
    * Function to save the latest ambience as .wav file.
    */
    public export(): void {
        if (this.generatedAmibienceUrl) {
            const a = document.createElement('a');
            a.href = this.generatedAmibienceUrl;
            a.download = `${this.audioCollection.name}-ambience.wav`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);

            URL.revokeObjectURL(this.generatedAmibienceUrl);
        }
    }

    public getLoadingStage(): string {
        switch (this.ambienceLoadingProgress) {
            case 0: return '1/6 Uploading files from the server...';
            case 1: return '2/6 Converting audio files...';
            case 2: return '3/6 Decoding audio files...';
            case 3: return '4/6 Combining audio files...';
            case 4: return '5/6 Applying audio files to wavesurfer...';
            case 5: return '6/6 Ambience loading finished';
            default: return '';
        }
    }
}
