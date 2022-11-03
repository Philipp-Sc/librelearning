
export function is_ready(){ 
  console.log("App is ready.");
  document.getElementById("center_text").remove();
}

// TODO: use web_sys ['AudioContext','AudioBufferSourceNode'] and js-sys instead
export function play_audio_from_uint8_array(uint8_array){ 

  console.log("Playing Audio"); 
 
  var context = new AudioContext();
  var buf;
  const gainNode = context.createGain();
  context.decodeAudioData(uint8_array.buffer, function(buffer) {
    buf = buffer;
    play();
  }); 
 
  function play() { 
    gainNode.gain.value = 1;
    var source = context.createBufferSource();
    source.buffer = buf; 
    source.connect(gainNode).connect(context.destination); 
    source.start(0);
  }   
}

/**
 * ebisu.js placed in assets:
 *  - assets/ebisu.min.es6.js (see https://raw.githubusercontent.com/fasiha/ebisu.js)
 * 
 * ./index.html edited to include:
 * 
 *  ```
 *  <link data-trunk rel="copy-file" href="assets/ebisu.min.es6.js" />
 * 
 *  <script type="text/javascript" src="ebisu.min.es6.js"></script>
 *  ```
 * 
 * TODO: Port EBISU to Rust
 */
export class SpacedRepetition {

    model_dict; 
    default_half_life_time;

    constructor() { 
        this.model_dict = {};  
        this.default_half_life_time = 1000*60*30;  
    }
 
    #transformEBISUModel(ebisu_model, prev_timestamp, timestamp, score){
      if(ebisu_model==null){ // score is null
        return [3,3,this.default_half_life_time]
      }else{
        var elapsedTime = timestamp - prev_timestamp;  // assertion: timestamp >= prev_timestamp  
        return ebisu.updateRecall(ebisu_model, score,1, elapsedTime);
      }
    }  
 
    #getEBISUModel(card_id, timestamps, scores){
      // load model
      var model = this.model_dict["EBISU_"+card_id+"_"+timestamps.length];
      
      if(!model){ 
        // first creation of model
        model = this.#transformEBISUModel(null,null,null,scores[0]); 
        for(var t=1;t<=timestamps.length-1;t++){ 
          model = this.#transformEBISUModel(
              model,
              timestamps[t-1],
              timestamps[t], 
              scores[t]);
        }
        // save model
        this.model_dict["EBISU_"+card_id+"_"+timestamps.length]=model;
      }
      return model;
    }

    #predictedHalfLifeEBISU(card_id,timestamps,scores,recall_probability,ebisu_model){
      var model = ebisu_model;
      if(model==null){
        model = this.#getEBISUModel(card_id,timestamps,scores)
      }
      return ebisu.modelToPercentileDecay(model, recall_probability, false, 1e-4);
    }

    /**
     * This function returns a date at which the card should be reviewed again.
     * The best usage of this is to decide the order of the cards to be reviewed. 
     * @param {*} card_id 
     * @param {*} timestamps = [t1,t2,t3,..,tn]
     * @param {*} scores = [s1,s2,s3,..,sn] 
     * @returns String 
     */
    calculateEBISU(card_id, timestamps, scores){   
      //console.assert(timestamps.length>0, timestamps);        
      //console.assert(scores.length>0, scores);     

      if (timestamps.length==0 || scores.length==0) {
        //console.log("returning new Date().getTime()");
        return new Date().getTime();
      }

      return new Date(
        timestamps[timestamps.length-1] + this.#predictedHalfLifeEBISU(card_id,timestamps,scores,0.5,null) // timeToElapseForHalfLife
         ).getTime();
    } 
}
     