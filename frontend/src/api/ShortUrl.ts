import axios from "axios";

export const shortUrl = async (url : string)=> {
    try{
        const response = await axios.post("http://127.0.0.1:8080/create", {
            url : url
        })
        const originalUrl = response.data.original_url;
        const shortUrl = response.data.short_url;
        return { originalUrl, shortUrl };
    }catch(error){  
        console.log("Error in Creating ShortUrl", error);
    }
}