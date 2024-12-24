import { UTApi } from "uploadthing/server";

export namespace UploadThingService {
  export const utapi = new UTApi();

  export const uploadFile = async (file: File) => {
    const res = await utapi.uploadFiles(file);
    if (res.error) throw new Error(res.error.message);
    return res.data.url;
  };
}
