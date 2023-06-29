import { ATS_ADMIN_KEY } from '$env/static/private';
import { invoke } from '@tauri-apps/api/tauri';

export function load( { cookies } ) {
    const atsCookie = cookies.get( 'atscookie' );
    cookies.set( 'atscookie', ATS_ADMIN_KEY, { path: '/' } );
    const foo = cookies.get( 'atscookie' );

    return {
        foo
    };

}