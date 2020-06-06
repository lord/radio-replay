export const getEastCoastTime = (timestamp) =>{
    const adjustedTimestamp = timestamp.toLocaleString("en-US", {
        timeZone: "America/New_York",
    });
    return adjustedTimestamp;
}