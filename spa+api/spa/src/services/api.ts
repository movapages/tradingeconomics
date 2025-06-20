// API service using RxJS fromFetch and BehaviorSubject
import { fromFetch } from 'rxjs/fetch';
import { BehaviorSubject, switchMap, catchError, of } from 'rxjs';

/**
 * Status subject and observable for /api/status endpoint.
 * Holds the latest status object (status, total_records, last_updated).
 */
const statusSubject = new BehaviorSubject<any>(null);
/** Observable for status updates. */
export const status$ = statusSubject.asObservable();
/**
 * Fetches /api/status and updates the status subject.
 */
export function refreshStatus() {
  fromFetch('/api/status')
    .pipe(
      switchMap((res: Response) => res.ok ? res.json() : of({ error: 'Failed to fetch status' })),
      catchError((err: any) => of({ error: err.message }))
    )
    .subscribe((data: any) => statusSubject.next(data));
}

/**
 * Raw data subject and observable for /api/raw endpoint.
 * Holds the full filtered dataset.
 */
const rawSubject = new BehaviorSubject<any>(null);
/** Observable for raw data updates. */
export const raw$ = rawSubject.asObservable();
/**
 * Fetches /api/raw and updates the raw data subject.
 */
export function refreshRaw() {
  fromFetch('/api/raw')
    .pipe(
      switchMap((res: Response) => res.ok ? res.json() : of({ error: 'Failed to fetch raw data' })),
      catchError((err: any) => of({ error: err.message }))
    )
    .subscribe((data: any) => rawSubject.next(data));
}

/**
 * Pie data subject and observable for /api/pie endpoint.
 * Holds pie chart data for the current dataset.
 */
const pieSubject = new BehaviorSubject<any>(null);
/** Observable for pie chart data updates. */
export const pie$ = pieSubject.asObservable();
/**
 * Fetches /api/pie and updates the pie data subject.
 */
export function refreshPie() {
  fromFetch('/api/pie')
    .pipe(
      switchMap((res: Response) => res.ok ? res.json() : of({ error: 'Failed to fetch pie data' })),
      catchError((err: any) => of({ error: err.message }))
    )
    .subscribe((data: any) => pieSubject.next(data));
}

/**
 * Import names subject and observable for /api/import-names endpoint.
 * Holds unique import names for the Import tab.
 */
const importNamesSubject = new BehaviorSubject<any>(null);
/** Observable for import names updates. */
export const importNames$ = importNamesSubject.asObservable();
/**
 * Fetches /api/import-names and updates the import names subject.
 */
export function refreshImportNames() {
  fromFetch('/api/import-names')
    .pipe(
      switchMap((res: Response) => res.ok ? res.json() : of({ error: 'Failed to fetch import names' })),
      catchError((err: any) => of({ error: err.message }))
    )
    .subscribe((data: any) => importNamesSubject.next(data));
}

/**
 * Export names subject and observable for /api/export-names endpoint.
 * Holds unique export names for the Export tab.
 */
const exportNamesSubject = new BehaviorSubject<any>(null);
/** Observable for export names updates. */
export const exportNames$ = exportNamesSubject.asObservable();
/**
 * Fetches /api/export-names and updates the export names subject.
 */
export function refreshExportNames() {
  fromFetch('/api/export-names')
    .pipe(
      switchMap((res: Response) => res.ok ? res.json() : of({ error: 'Failed to fetch export names' })),
      catchError((err: any) => of({ error: err.message }))
    )
    .subscribe((data: any) => exportNamesSubject.next(data));
}

// Initial fetch for status, raw, and pie only
refreshStatus();
refreshRaw();
refreshPie(); 